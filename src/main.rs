#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{fs, io};
use std::collections::HashMap;
use std::process::exit;

use roxmltree;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// atenaCXMLconv: Convert Atena 26 Contact XML into CSV
pub struct atenaCXMLconv {
    /// XML file paths
    #[structopt(name = "XMLpath")]
    pub paths: String,
}


fn main() {
    let opt = atenaCXMLconv::from_args();

    if opt.paths.len() == 0 {
        eprintln!("no contact XML file specified");
        let mut out = io::stderr();
        let mut app = atenaCXMLconv::clap();

        // show long help and exit
        app.write_long_help(&mut out).expect("failed to write to stderr");
        exit(1);
    }

    let ret = fs::read_to_string(&opt.paths);
    match ret {
        Ok(s) => {
            parse_xml_string(&s);
        }
        Err(E) => {
            // read error / utf8 error
            eprintln!("{}", E);
            exit(1);
        }
    }
}

fn parse_xml_string(s: &String) {
    let headers = ["LastName", "FirstName", "furiLastName", "furiFirstName", "AddressCode", "FullAddress",
        "Suffix", "PhoneItem", "EmailItem", "Memo", "NamesOfFamily1", "X-Suffix1", "NamesOfFamily2", "X-Suffix2", "NamesOfFamily3", "X-Suffix3",
        "atxBaseYear", "X-NYCardHistory"];

    // print above array join by ","
    println!("{},", headers.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    let doc = roxmltree::Document::parse(s).unwrap();
    let nodes = doc.root_element().first_element_child().filter(|n| n.node_type() == roxmltree::NodeType::Element);

    let mut node_maybe = nodes;
    while let Some(node) = node_maybe
    {
        let mut scores = HashMap::new();
        for base in node.children() {
            if !base.is_element() { continue; }
            // elementそれぞれに応じて解析し、hashmapにぶちこむ
            match base.tag_name().name() {
                "PersonName" => {
                    if let Some(pni) = base.first_element_child() {
                        if let Some(pnfull) = pni.first_element_child() {
                            if let Some(firstn) = pnfull.next_sibling_element() {
                                scores.insert("FirstName".to_string(), firstn.text().unwrap());
                                scores.insert("furiFirstName".to_string(), firstn.attribute("pronunciation").unwrap_or(""));
                                if let Some(secondn) = firstn.next_sibling_element() {
                                    scores.insert("LastName".to_string(), secondn.text().unwrap());
                                    scores.insert("furiLastName".to_string(), secondn.attribute("pronunciation").unwrap_or(""));
                                }
                            }
                        }
                    }
                }
                "Address" => {
                    if let Some(addritem) = base.first_element_child() {
                        if let Some(addrcode) = addritem.first_element_child() {
                            scores.insert("AddressCode".to_string(), addrcode.text().unwrap_or(""));
                            if let Some(fulladdr) = addrcode.next_sibling_element() {
                                scores.insert("FullAddress".to_string(), fulladdr.text().unwrap_or(""));
                            }
                        }
                    }
                }
                "Phone" => {
                    if let Some(phoneitem) = base.first_element_child() {
                        scores.insert("PhoneItem".to_string(), phoneitem.text().unwrap_or(""));
                    }
                }
                "Email" => {
                    if let Some(emailitem) = base.first_element_child() {
                        scores.insert("EmailItem".to_string(), emailitem.text().unwrap_or(""));
                    }
                }
                "Extension" => {
                    let mut namesoffamily = 1;  // NamesOfFamily1, NamesOfFamily2, ...
                    for extItems in base.children() {
                        if extItems.has_attribute("name") {
                            let attrname = extItems.attribute("name").unwrap_or("").to_string().to_owned();
                            let exti = extItems.text().unwrap_or("");

                            if attrname == "NamesOfFamily" {
                                // NamesOfFamily は何回も出現するがそれぞれ別に割り振る、一方で X-Suffixにはなぜか順番に番号がついている
                                scores.insert(format!("{}{}", attrname, namesoffamily), exti);
                                namesoffamily += 1;
                            } else {
                                scores.insert(attrname, exti);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // ヘッダのハッシュマップと同じ順番にぶちこんだデータを表示しカンマで区切る, ￥nは消す
        let mut cnt = 0;
        while cnt < 18 {
            let ss = scores.get(headers[cnt])
                .unwrap_or(&&*"".to_string())
                .replace('\n', "");
            print!("{}", ss);
            if cnt < 17 {
                print!(",");
            }
            cnt += 1;
        }
        println!();

        node_maybe = node.next_sibling_element();
    }
}

