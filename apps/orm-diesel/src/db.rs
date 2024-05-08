/*
 * MIT License
 *
 * Copyright (c) 2023 tomoncle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::env;
use std::sync::Once;

use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, HandleEvent, PooledConnection};
use diesel::r2d2::event::{AcquireEvent, CheckinEvent, CheckoutEvent, ReleaseEvent, TimeoutEvent};
use dotenvy::dotenv;
use log::{debug, error, info};

#[derive(Debug)]
struct PGEventHandler;

impl HandleEvent for PGEventHandler {
    fn handle_acquire(&self, event: AcquireEvent) {
        debug!("acquire  connection： {:?}", event);
    }

    fn handle_release(&self, event: ReleaseEvent) {
        debug!("release  connection： {:?}", event);
    }

    fn handle_checkout(&self, event: CheckoutEvent) {
        debug!("checkout connection： {:?}", event);
    }

    fn handle_timeout(&self, event: TimeoutEvent) {
        debug!("connection  timeout： {:?}", event);
    }

    fn handle_checkin(&self, event: CheckinEvent) {
        debug!("checkin  connection： {:?}", event);
    }
}


static INIT: Once = Once::new();
static mut POOL: Option<r2d2::Pool<ConnectionManager<PgConnection>>> = None;

fn db_pool() -> &'static r2d2::Pool<ConnectionManager<PgConnection>> {
    unsafe {
        INIT.call_once(|| {
            info!("initial load connection pool.");
            dotenv().ok(); // 读取 .env 文件
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            POOL = Some(r2d2::Pool::builder()
                .event_handler(Box::new(PGEventHandler))
                .max_size(10)
                .min_idle(Some(5))
                .build(manager).unwrap());
        });
        POOL.as_ref().unwrap()
    }
}


pub fn db_conn() -> Result<PooledConnection<ConnectionManager<PgConnection>>, anyhow::Error> {
    db_pool().get().map_err(|err| {
        error!("获取数据库连接失败: {:?}", err);
        anyhow::Error::from(err)
    })
}

