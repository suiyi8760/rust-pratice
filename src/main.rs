
mod demo_match;
mod demo_condition;
mod demo_html2md;
mod demo_fib;

use crate::demo_match::demo_match;
use crate::demo_condition::demo_condition;
use crate::demo_html2md::demo_html2md;
use crate::demo_fib::fib_pre;

fn main() {
    println!("match模式匹配--------------------------------");
    demo_match();
    println!("条件控制--------------------------------");
    demo_condition();
    println!("html2md(简单体验rust开发流程)--------------------------------");
    demo_html2md();
    println!("斐波那契数列--------------------------------");
    println!("{}",fib_pre(10));
}
