//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
use serde_json::{json, Value};

#[test]
fn test11() {
    let body_str = r#")]}'\n\n2838\n[["wrb.fr","MkEWBc","[[null,null,null,[[[0,[[[null,134]],[true]]],[1,[[[null,135],[135,307]],[false,true]]],[2,[[[null,308],[308,478]],[false,true]]],[3,[[[null,479],[479,623]],[false,true]]],[4,[[[null,624],[624,776]],[false,true]]]],776]],[[[null,\"Dāng nín qiánrù gèng xiānjìn de RUST jiǎoluò shí, zhòngyào de shì, nín quèbǎo nín duì jīběn miàn yǒu suǒ liǎojiě. Zài Rust zhōng, rú zài rènhé biānchéng yǔyán zhōng, gè zhǒng guānjiàn zì hé gàiniàn de jīngquè hányì dōu biàn dé zhòngyào, yīnwèi nín kāishǐ yǐ gèng fùzá de fāngshì shǐyòng gāi yǔyán. Zài běnzhāng zhōng, wǒmen jiāng zǒuguò xǔduō shēng xiù de yuán yǔ, bìng chángshì gèng qīngchǔ dì dìngyì tāmen de yìsi, yǐjí tāmen rúhé gōngzuò, yǐjí wèishéme tāmen zhèng shì tāmen suǒ chǔ de fāngshì. Jùtǐ lái shuō, wǒmen jiāng chákàn biànliàng hé zhí de bùtóng, tāmen rúhé zài nèicún zhōng biǎoshì, yǐjí chéngxù jùyǒu bùtóng de nèicún qūyù. Ránhòu, wǒmen jiāng tǎolùn suǒyǒuquán de yīxiē wéimiào zhī chù, jièyòng hé shòumìng, yǐbiàn zài nín jìxù běn shū zhīqián nín xūyào zhǎngwò.\",null,null,null,[[\"当您潜入更先进的RUST角落时，重要的是，您确保您对基本面有所了解。\",null,null,null,[[\"当您潜入更先进的RUST角落时，重要的是，您确保您对基本面有所了解。\",[5]],[\"当你潜入锈病的更先进的角落，它是你确保你有基本面的一个坚实的理解是很重要的。\",[1]]]],[\"在Rust中，如在任何编程语言中，各种关键字和概念的精确含义都变得重要，因为您开始以更复杂的方式使用该语言。\",null,true,null,[[\"在Rust中，如在任何编程语言中，各种关键字和概念的精确含义都变得重要，因为您开始以更复杂的方式使用该语言。\",[5]],[\"拉斯特，在任何编程语言，不同的关键字和概念的确切含义当你开始使用更复杂的方式的语言就变得很重要。\",[1]]]],[\"在本章中，我们将走过许多生锈的原语，并尝试更清楚地定义他们的意思，以及他们如何工作，以及为什么他们正是他们所处的方式。\",null,true,null,[[\"在本章中，我们将走过许多生锈的原语，并尝试更清楚地定义他们的意思，以及他们如何工作，以及为什么他们正是他们所处的方式。\",[5]],[\"在本章中，我们将通过多种锈病的原语的行走，并试图更清晰地WHA他们的意思定义，他们是如何工作的，以及为什么他们正是他们的方式。\",[1]]]],[\"具体来说，我们将查看变量和值的不同，它们如何在内存中表示，以及程序具有不同的内存区域。\",null,true,null,[[\"具体来说，我们将查看变量和值的不同，它们如何在内存中表示，以及程序具有不同的内存区域。\",[5]],[\"具体而言，我们将看看变量和值如何不同，他们是如何在内存中表示，一个程序有不同的内存区域。\",[1]]]],[\"然后，我们将讨论所有权的一些微妙之处，借用和寿命，以便在您继续本书之前您需要掌握。\",null,true,null,[[\"然后，我们将讨论所有权的一些微妙之处，借用和寿命，以便在您继续本书之前您需要掌握。\",[5]],[\"然后，我们将讨论一些所有权，借款和寿命的精妙之处在于你需要对你继续与前书的句柄。\",[1]]]]]]],\"zh-CN\",1,\"en\",[\"As you dive into the more advanced corners of Rust, it’s important that you ensure you have a solid understanding of the fundamentals. In Rust, as in any programming language, the precise meaning of various keywords and concepts becomes important as you begin to use the language in more sophisticated ways. In this chapter, we’ll walk through many of Rust’s primitives and try to define more clearly wha they mean, how they work, and why they are exactly the way that they are. Specifically, we’ll look at how variables and values differ, how they are represented in memory, and the different memory regions a program has. We’ll then discuss some of the subtleties of ownership, borrowing, and lifetimes that you’ll need to have a handle on before you continue with the book.\",\"en\",\"zh-CN\",true]],\"en\"]",null,null,null,"generic"]]\n57\n[["di",1104],["af.httprm",1104,"-4956069835660609852"]]\n26\n[["e",4,null,null,4482]]\n'"#;
    let n1 = &body_str[6..].find(|c: char| c == '\n').unwrap();
    let n2 = (n1 + 6) as usize;
    let f = *&body_str[6..n2].parse::<usize>().unwrap();
    let content = &body_str[n2..f];

    println!("{:#?}", content);

    assert_eq!(1 + 1, 2);
}
