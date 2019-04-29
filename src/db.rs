use chrono::{DateTime, Local};
use postgres::{rows::Row, Connection, TlsMode};

#[derive(Debug)]
pub struct Proxy {
    pub insert: bool,
    pub update: bool,
    pub work: bool,
    pub anon: bool,
    pub checks: i32,
    pub hostname: String,
    pub host: String,
    pub port: String,
    pub scheme: String,
    pub create_at: DateTime<Local>,
    pub update_at: DateTime<Local>,
    pub response: i64,
}

fn full_from_row(row: Row) -> Result<Proxy, String> {
    Ok(Proxy {
        insert: false,
        update: false,
        work: row
            .get_opt(0)
            .ok_or_else(|| "error get work")?
            .map_err(|e| format!("error unwrap work {}", e))?,
        anon: row
            .get_opt(1)
            .ok_or_else(|| "error get anon")?
            .map_err(|e| format!("error unwrap anon {}", e))?,
        checks: row
            .get_opt(2)
            .ok_or_else(|| "error get checks")?
            .map_err(|e| format!("error unwrap checks {}", e))?,
        hostname: row
            .get_opt(3)
            .ok_or_else(|| "error get hostname")?
            .map_err(|e| format!("error unwrap hostname {}", e))?,
        host: row
            .get_opt(4)
            .ok_or_else(|| "error get host")?
            .map_err(|e| format!("error unwrap host {}", e))?,
        port: row
            .get_opt(5)
            .ok_or_else(|| "error get port")?
            .map_err(|e| format!("error unwrap port {}", e))?,
        scheme: row
            .get_opt(6)
            .ok_or_else(|| "error get scheme")?
            .map_err(|e| format!("error unwrap scheme {}", e))?,
        create_at: row
            .get_opt(7)
            .ok_or_else(|| "error get create_at")?
            .map_err(|e| format!("error unwrap create_at {}", e))?,
        update_at: row
            .get_opt(8)
            .ok_or_else(|| "error get update_at")?
            .map_err(|e| format!("error unwrap update_at {}", e))?,
        response: row
            .get_opt(9)
            .ok_or_else(|| "error get response")?
            .map_err(|e| format!("error unwrap response {}", e))?,
    })
}

pub fn get_connection(params: &str) -> Connection {
    Connection::connect(params, TlsMode::None).unwrap()
}

pub fn get_all_proxy(conn: Connection) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies",
            &[],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}

pub fn get_n_proxy(conn: Connection, n: i64) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
        LIMIT
            $1",
            &[&n],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}

pub fn get_all_work_proxy(conn: Connection) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true",
            &[],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}

pub fn get_n_work_proxy(conn: Connection, n: i64) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true
        LIMIT
            $1",
            &[&n],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}

pub fn get_all_work_anon_proxy(conn: Connection) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true AND anon = true",
            &[],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}

pub fn get_all_old_proxy(conn: Connection) -> Result<Vec<Proxy>, String> {
    let mut proxies = Vec::new();
    let rows = &conn
        .query(
            "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true OR update_at < NOW() - (INTERVAL '3 days') * checks",
            &[],
        )
        .map_err(|e| format!("error query {}", e.to_string()))?;
    for row in rows {
        proxies.push(full_from_row(row)?);
    }
    Ok(proxies)
}