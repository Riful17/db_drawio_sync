use chrono::Local;
use log::*;
use sqlparser::dialect;
use sqlparser::parser::Parser;
use std::fs::File;
use std::io::{Write};
use xmltree::Element;

fn main() {
    let data: &str = r##"
<mxfile host="Electron" modified="2024-01-02T23:57:44.669Z" agent="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) draw.io/22.1.2 Chrome/114.0.5735.289 Electron/25.9.4 Safari/537.36" etag="VsMo5dle4Q90sCNxzw9a" version="22.1.2" type="device">
  <diagram name="Page-1" id="a7904f86-f2b4-8e86-fa97-74104820619b">
    <mxGraphModel dx="954" dy="616" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="1100" pageHeight="850" background="none" math="0" shadow="0">
      <root>

        <mxCell id="0" />
        <mxCell id="1" parent="0" />

        <mxCell id="table.entrances.header" value="entrances" style="swimlane;fontStyle=0;childLayout=stackLayout;horizontal=1;startSize=30;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;whiteSpace=wrap;html=1;expand=0;" parent="1" vertex="1">
          <mxGeometry width="330" height="120" as="geometry" />
        </mxCell>

        <mxCell id="table.entrances.field.id" value="`id` [PK] bigint(20) NOT NULL AUTO_INCREMENT" style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=4;spacingRight=4;overflow=hidden;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;rotatable=0;whiteSpace=wrap;html=1;" parent="table.entrances.header" vertex="1">
          <mxGeometry y="30" width="330" height="30" as="geometry" />
        </mxCell>

        <mxCell id="table.entrances.field.uuid" value="`uuid` uuid DEFAULT NULL" style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=4;spacingRight=4;overflow=hidden;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;rotatable=0;whiteSpace=wrap;html=1;" parent="table.entrances.header" vertex="1">
          <mxGeometry y="60" width="330" height="30" as="geometry" />
        </mxCell>

        <mxCell id="table.entrances.field.created_at" value="`created_at` datetime(3) DEFAULT NULL" style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=4;spacingRight=4;overflow=hidden;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;rotatable=0;whiteSpace=wrap;html=1;" parent="table.entrances.header" vertex="1">
          <mxGeometry y="90" width="330" height="30" as="geometry" />
        </mxCell>

      </root>
    </mxGraphModel>
  </diagram>
</mxfile>
"##;

    let mut names_element = Element::parse(data.as_bytes()).unwrap();

    println!("{:#?}", names_element);
    {
        // get first `name` element
        // let name = names_element
        //     .get_mut_child("name")
        //     .expect("Can't find name element");
        // name.attributes.insert("suffix".to_owned(), "mr".to_owned());
    }

    names_element
        .write_with_config(
            File::create("../result.drawio").unwrap(),
            xmltree::EmitterConfig::new()
                .write_document_declaration(false)
                .perform_indent(true),
        )
        .expect("TODO: panic message");

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
        })
        .init();

    debug!("debug line");

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

    let dialect = dialect::GenericDialect {};
    let res = Parser::parse_sql(&dialect, sql_comment).unwrap();

    for statement in res {
        if let sqlparser::ast::Statement::CreateTable { name, columns, .. } = statement {
            for each_name in name.0 {
                debug!("Table each_name: {:?}", each_name.value);
            }
            // debug!("Table: {:?}", name);
            for column_def in columns {
                debug!(
                    "Column: {}, Type: {}",
                    column_def.name.value, column_def.data_type
                );
                for opt in &column_def.options {
                    if opt.name.is_none() {
                        continue;
                    }
                    debug!(
                        "  Option: {:?} = {:?}",
                        opt.name.clone().unwrap().value,
                        opt.option
                    );
                }
            }
        }
    }
}
