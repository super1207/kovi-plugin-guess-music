mod runpy;

use kovi::{bot::runtimebot::kovi_api::KoviApi, log, tokio, Message, PluginBuilder as plugin};
use runpy::run_virtual_python;
use std::path::PathBuf;


#[macro_use]
extern crate lazy_static; 


const PLUS_NAME:&str = "KOVI_PLUGIN_GUESS_MUSIC";

lazy_static! {
    static ref G_PATH:std::sync::RwLock<Option<PathBuf>>  = std::sync::RwLock::new(None);
}

const MY_CODE:&str = r#"
try:
	from openai import OpenAI
except:
	red_install("openai")
	from openai import OpenAI
import json
# 我的API很慢的啦，如果你有自己的API，可以自己修改这里
client = OpenAI(
    api_key="sk-guess_the_music_kovi",
    base_url="https://sgpt.super1207.top/v1",
)
def send_messages(messages):
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=messages,
        tools=tools,
        temperature=0.8
    )
    return response.choices[0].message
tools = [
    {
        "type": "function",
        "function": {
            "name": "get_music_info",
            "description": "根据歌手和歌名获取一首歌的信息",
            "parameters": {
                "type": "object",
                "properties": {
                    "singer": {
                        "type": "string",
                        "description": "歌手，如：周杰伦",
                    },
                    "name": {
                        "type": "string",
                        "description": "歌名，如：青花瓷",
                    }
                },
                "required": ["name","singer"]
            },
        }
    },
]
def get_music_name(msg):
    messages = []
    messages.append({"role": "user", "content": msg})
    message = send_messages(messages)
    # print(message)
    if message.tool_calls != None and message.tool_calls[0].function.name == "get_music_info":
        js = json.loads(message.tool_calls[0].function.arguments)
        name = js["name"]
        singer = js["singer"]
        return f'{singer}-{name}'
    return '分析失败'
ret = get_music_name(red_in())
red_out(ret)
"#;


fn need_deal(str0:&str) -> bool {
    if !str0.starts_with("#猜歌") {
        return false;
    }
    return true;
}

fn deal_str0(str0:&str,app_dir:PathBuf) -> Result<(String,String), Box<dyn std::error::Error>> {
    let key_word = &str0[7..].trim();
    log::debug!("[{PLUS_NAME}]:key_word:{key_word}");
    // 此函数创建的环境详见：https://super1207.github.io/redreply/#/detailref/?id=%e8%bf%90%e8%a1%8cpy
    // code是需要运行的python代码
    // key_word是用户输入的关键词
    // app_dir是python的运行目录，会影响python里面的当前目录
    // app_flag是当前插件的唯一标记，用于区分不同的插件创建不同的虚拟环境
    let ret = run_virtual_python(MY_CODE, key_word, app_dir, PLUS_NAME)?;
    return Ok((key_word.to_string(),ret));
}

#[kovi::plugin]
async fn main() {
    
    // 获得插件目录
    {
        let bot = plugin::get_runtime_bot();
        let tmp_dir_t2 = bot.get_data_path();
        let mut lk = G_PATH.write().unwrap();
        *lk = Some(tmp_dir_t2);

    }

    plugin::on_msg(|event| async move {

        // 取文本
        let str0_opt = event.borrow_text();
        let str0_ref;
        if let Some(str0) = str0_opt {
            str0_ref = str0;
        } else {
            return;
        }
        
        // 判断是否需要处理
        if !need_deal(str0_ref) {
            return;
        }

        // 获得插件目录
        let app_dir = {
            let lk =  G_PATH.read().unwrap();
            lk.to_owned().unwrap()
        };
        
        // 拷贝一份
        let str0 = str0_ref.to_string();

        // 得到处理结果
        let deal_ret_rst = tokio::task::spawn_blocking(move || {
            match deal_str0(&str0,app_dir) {
                Ok(ret) => {
                   return Some(ret);
                },
                Err(err) => {
                    log::error!("[{PLUS_NAME}] error1:{err}");
                    return None;
                }
            }
        }).await;

         // 不知道在干嘛，反正这样就编译过了
        let deal_ret = match deal_ret_rst {
            Ok(deal_ret_t) => {
                deal_ret_t
            },
            Err(err) => {
                log::error!("[{PLUS_NAME}] error2:{err}");
                None
            },
        };

        // 发出去！
        if let Some((key,anwser)) = deal_ret {
            let msg = Message::new();
            let msg = msg.add_reply(event.message_id);
            let msg = msg.add_text(format!("问题：{key}\r\n"));
            let msg = msg.add_text(format!("答案：{anwser}\r\n"));
            // let msg = msg.add_text(format!("powered by kovi"));
            event.reply(msg);
        }
    });
}
