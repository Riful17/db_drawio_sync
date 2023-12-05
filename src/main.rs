use sqlparser::dialect;
use sqlparser::parser::Parser;
use chrono::Local;
use log::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let target = Box::new(File::create("app.log").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args(),
            )
        }).init();

    debug!("debug line");

    info!("Hello world");
    info!("Hello world");
    info!("Hello world");
    info!("Hello world");

    let sql_comment = "/*
 Navicat Premium Data Transfer

 Source Server         : VM_MariaDB
 Source Server Type    : MariaDB
 Source Server Version : 110003 (11.0.3-MariaDB)
 Source Host           : 94.19.108.70:3306
 Source Schema         : catalog

 Target Server Type    : MariaDB
 Target Server Version : 110003 (11.0.3-MariaDB)
 File Encoding         : 65001

 Date: 05/12/2023 00:51:28
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for entrances
-- ----------------------------
DROP TABLE IF EXISTS `entrances`;
CREATE TABLE `entrances` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `uuid` uuid DEFAULT NULL,
  `number` varchar(255) DEFAULT NULL COMMENT 'just some comment',
  `description` varchar(255) DEFAULT NULL,
  `house_id` bigint(20) DEFAULT NULL,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `external_uuid` uuid DEFAULT NULL,
  `address` varchar(255) DEFAULT NULL,
  `count_floor` bigint(20) DEFAULT NULL,
  `count_flat` bigint(20) DEFAULT NULL,
  `count_pantries` bigint(20) DEFAULT NULL,
  `count_elevator` bigint(20) DEFAULT NULL,
  `count_ventilation` bigint(20) DEFAULT NULL,
  `count_fire_safety` bigint(20) DEFAULT NULL,
  `count_intercom` bigint(20) DEFAULT NULL,
  `count_camera` bigint(20) DEFAULT NULL,
  `count_controller` bigint(20) DEFAULT NULL,
  `count_sensor` bigint(20) DEFAULT NULL,
  `count_counter` bigint(20) DEFAULT NULL,
  `count_router` bigint(20) DEFAULT NULL,
  `count_subscriber` bigint(20) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=152 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

SET FOREIGN_KEY_CHECKS = 1;
";



    let _sql = "DROP TABLE IF EXISTS `access_keys`;
CREATE TABLE `access_keys` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `uuid` uuid DEFAULT NULL,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `enter_type` varchar(256) DEFAULT NULL,
  `comment` varchar(256) DEFAULT NULL,
  `alias` varchar(256) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Table structure for account_config
-- ----------------------------
DROP TABLE IF EXISTS `account_config`;
CREATE TABLE `account_config` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `uuid` uuid DEFAULT NULL,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `account_id` varchar(256) DEFAULT NULL,
  `account_config_id` varchar(256) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1901 DEFAULT CHARSET=utf8mb4;
";

    let dialect = dialect::GenericDialect{};
    let res = Parser::parse_sql(&dialect, sql_comment).unwrap();

    for statement in res {
        match statement {
            sqlparser::ast::Statement::CreateTable { name, columns, .. } => {
                for each_name in name.0 {
                    debug!("Table each_name: {:?}", each_name.value);
                }
                // debug!("Table: {:?}", name);
                for column_def in columns {
                    debug!("Column: {}, Type: {}", column_def.name.value, column_def.data_type);
                    for opt in &column_def.options {
                        if opt.name.is_none() {
                            continue;
                        }
                        debug!("  Option: {:?} = {:?}", opt.name.clone().unwrap().value, opt.option);
                    }
                }
            }
            _ => {}
        }
    }}