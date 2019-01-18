use super::spider::proxy_getter::FUNCS;
use crate::AProxyPool;
use log::{error, info};

/// 爬虫线程
pub fn spider_thread(proxies: AProxyPool) {
    info!("代理爬取开始");
    for func in &FUNCS {
        let ret = func().unwrap_or_else(|err| {
            error!("{:?}", err);
            vec![]
        });
        let mut proxies = proxies.lock().expect("spider_thread: 无法获取锁");
        proxies.extend_unstable(ret);
    }
    info!("代理爬取结束");
}