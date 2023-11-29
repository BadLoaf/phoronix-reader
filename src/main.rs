// phoronix reader
// Copyright (C) 2023  Avijya Shrestha
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate color_eyre;
extern crate colored;
extern crate reqwest;
extern crate select;
extern crate textwrap;

use colored::*;

mod article;
use article::Article;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Choose between gui or cli
    #[arg(short, long, default_value = "true")]
    term: Option<bool>,
}

fn main() {
    let cli = Cli::parse();
    if Some(true) == cli.term {
        println!("cli mode: {}", cli.term.unwrap());
        let phoronix_articles = Article::get_articles();
        for i in phoronix_articles {
            println!("Title:        {}", i.title.green().bold());
            println!("Link:         https://www.phoronix.com{}/", i.link);
            println!("Description:  \n{}", i.summary);
            println!("");
        }
    } else {
        println!("Sorry, Gui under construction!");
    }
    // println!("{:?}", phoronix_articles);
}
