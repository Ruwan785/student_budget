use std::io::{self, Write};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

struct Expense {
    category: String,
    actual: f64,
}

fn ask_float(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("❌ Please enter a valid number."),
        }
    }
}

fn get_expenses_from_user() -> Vec<Expense> {
    let categories = vec![
        "Housing",
        "Utilities",
        "Food",
        "Transportation",
        "Healthcare / Insurance / Jym Membership",
        "Clothing / Education and other",
        "Taxes (no tax under 540 Euros)",
    ];

    let mut expenses = Vec::new();

    println!("\n📤 Please enter your monthly expenses:");

    for category in categories {
        let actual = ask_float(&format!("💰 How much did you spend on {} (€)? ", category));
        expenses.push(Expense {
            category: category.to_string(),
            actual,
        });
    }

    expenses
}

fn generate_pdf_report(gross_income: f64, total_expenses: f64, savings: f64, expenses: &[Expense]) {
    let (doc, page1, layer1) = PdfDocument::new("Student Financial Summary", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let mut y = Mm(280.0);
    let step = Mm(10.0);

    current_layer.use_text("📊 Student Financial Summary", 18.0, Mm(20.0), y, &font);
    y -= step;
    current_layer.use_text(format!("Gross Income: €{:.2}", gross_income), 12.0, Mm(20.0), y, &font);
    y -= step;
    current_layer.use_text(format!("Total Expenses: €{:.2}", total_expenses), 12.0, Mm(20.0), y, &font);
    y -= step;
    current_layer.use_text(format!("Monthly Savings: €{:.2}", savings), 12.0, Mm(20.0), y, &font);
    y -= step;

    current_layer.use_text("Expenses Breakdown:", 14.0, Mm(20.0), y, &font);
    y -= step;

    for e in expenses {
        current_layer.use_text(
            format!("- {:<40} €{:.2}", e.category, e.actual),
            10.0,
            Mm(20.0),
            y,
            &font,
        );
        y -= Mm(8.0);
    }

    let file = File::create("financial_summary.pdf").unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();
    println!("📄 PDF saved as: financial_summary.pdf");
}

fn main() {
    println!("💼 Student Monthly Income & Expense Tracker");

    let hours = ask_float("⏱ How many hours did you work this month? ");
    let wage = ask_float("💶 What is your hourly wage (€)? ");
    let gross_income = hours * wage;

    println!("\n📥 Gross Income: €{:.2}", gross_income);

    let expenses = get_expenses_from_user();
    let total_expenses: f64 = expenses.iter().map(|e| e.actual).sum();
    let savings = gross_income - total_expenses;

    println!("\n📋 Summary:");
    println!("Gross Income     : €{:.2}", gross_income);
    println!("Total Expenses   : €{:.2}", total_expenses);
    println!("💰 Monthly Savings: €{:.2}", savings);

    println!("\n📊 Percentage Breakdown:");
    for e in &expenses {
        let percent = if gross_income > 0.0 {
            (e.actual / gross_income) * 100.0
        } else {
            0.0
        };
        println!("- {:<50} {:>5.1}%", e.category, percent);
    }

    let savings_percent = if gross_income > 0.0 {
        (savings / gross_income) * 100.0
    } else {
        0.0
    };
    println!("- Savings{:>47} {:>5.1}%", "", savings_percent);

    generate_pdf_report(gross_income, total_expenses, savings, &expenses);
}

