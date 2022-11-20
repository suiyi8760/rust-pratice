
mod demo_match;
mod demo_condition;
mod demo_html2md;

use crate::demo_match::demo_match;
use crate::demo_condition::demo_condition;
use crate::demo_html2md::demo_html2md;

fn main() {
    println!("match模式匹配--------------------------------");
    demo_match();
    println!("条件控制--------------------------------");
    demo_condition();
    println!("html2md(简单体验rust开发流程)--------------------------------");
    demo_html2md();
}
