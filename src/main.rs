mod version;

use std::fs::File;
use std::io::Write;
use git2::{Repository, Signature, Error, RemoteCallbacks, Cred, PushOptions};
use std::path::Path;
use structopt::StructOpt;
use crate::version::DwVersion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    println!("参数列表:{:?}", &args);
    // 打开 Git 仓库
    let repo = Repository::open(Path::new(&args.path))?;

    // 获取当前 HEAD 的 commit
    let head = repo.head()?;
    let commit = repo.find_commit(head.target().unwrap())?;
    let tags = repo.tag_names(None)?;

    let mut version: DwVersion = match args.tag {
        None => {
            // 找到最后一个标签
            let last_tag = tags
                .iter()
                .filter(|tag|DwVersion::is_valid_version(tag.unwrap()))
                .flat_map(|tag| DwVersion::parse(tag.unwrap()))
                .max_by(|a, b| a.cmp(b));
          let mut version =  last_tag.unwrap_or_else(|| DwVersion::parse("v1.0.0").unwrap());
            // 解析最后一个标签的版本号
            println!("version: {:?}", version);
            version = version.plus_patch();
            version.set_pre(format!("{}", args.env));
            version.auto_set_build();
            version
        }
        Some(tag) => {
           let find= tags
                .iter()
                .flat_map(|tag| DwVersion::parse(tag.unwrap()))
                .find(|v| v.to_string() == tag);
            match find {
                None => {
                    DwVersion::from(tag.as_str())
                }
                Some(_) => {
                    let v = DwVersion::from(tag.as_str());
                    panic!("tag已存在,请重新输入: {}", v);
                }
            }
        }
    };

    // 构建新版本号
    let new_version = version.to_string();

    // 输出新版本号
    println!("新版本号为: {}", new_version);
    write_version(&repo, new_version.clone())?;
    // 打标签
    tag_commit(&repo, &commit, &new_version, &args.message.as_str(), &args.username.as_str(), &args.email.as_str())?;


    // 推送标签到远程仓库（例如 GitLab）
    push_tag(&repo, &new_version, &args.remote.as_str(), &args.username.as_str(), &args.access_token.as_str())?;

    Ok(())
}

fn write_version(repo: &Repository, version: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = repo.path().parent().unwrap().join("VERSION");
    let mut file = File::options().create(true)
        .write(true)
        .open(path).unwrap();
    file.write(version.as_bytes())?;
    // 获取 Git 索引
    Ok(())
}

fn tag_commit(repo: &Repository, commit: &git2::Commit, tag_name: &str, tag_message: &str, username: &str, email: &str) -> Result<(), Error> {
    // 获取签名
    let signature = Signature::now(username, email)?;

    // 创建标签
    repo.tag(tag_name, commit.as_object(), &signature, tag_message, true)?;

    Ok(())
}

fn push_tag(repo: &Repository, tag_name: &str, remote_name: &str, username: &str, password: &str) -> Result<(), Error> {
    // 获取远程仓库名称（例如 "origin"）
    // 设置身份验证回调
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext(username, password)
    });
    let mut remote = repo.find_remote(remote_name)?;
    // 配置远程仓库使用回调
    // 获取远程仓库
    let mut options = PushOptions::new();
    let opt = options.remote_callbacks(callbacks);
    // 推送标签到远程仓库
    remote.push(&[format!("refs/tags/{}", tag_name)], Option::from(opt))?;

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "gittag", about = "git的tag管理工具")]
struct Cli {
    #[structopt(short = "p", long = "path", default_value = ".", help = "仓库路径")]
    path: String,
    #[structopt(short = "t", long = "tag", help = "tag名称")]
    tag: Option<String>,
    #[structopt(short = "m", long = "message", default_value = "自动tag发布", help = "tag信息")]
    message: String,
    #[structopt(short = "e", long = "env", default_value = "Release", help = "打包的环境后缀")]
    env: String,
    #[structopt(short = "r", long = "remote", default_value = "origin", help = "远程仓库名称")]
    remote: String,
    #[structopt(short = "u", long = "username", help = "账号")]
    username: String,
    #[structopt(short = "a", long = "access_token", help = "密码")]
    access_token: String,
    #[structopt(short = "em", long = "email", default_value = "xiaozhiyun@163.com", help = "邮箱")]
    email: String,
}

#[cfg(test)]
mod testing {
    use std::path::Path;
    use git2::Repository;
    use crate::write_version;

    #[test]
    pub fn test_version_path() {
        let repo = Repository::open(Path::new(".")).unwrap();
        write_version(&repo, "v1.0.0".to_string()).unwrap();
    }
}
