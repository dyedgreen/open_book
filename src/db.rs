use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Book {
    conn: Connection,
}

#[derive(Debug)]
pub struct Equation {
    description: String,
    equation: String,
}

impl Book {
    pub fn open(file: &str) -> Result<Book> {
        let book = Book { conn: Connection::open(&file)? };
        book.conn.execute(r#"
            CREATE TABLE IF NOT EXISTS equations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                equation TEXT NOT NULL
            )
        "#, params!())?;
        Ok(book)
    }

    pub fn add_equation(&self, description: &str, equation: &str) -> Result<()> {
        self.conn.execute(r#"
            INSERT INTO equations (description, equation) VALUES (?, ?)
        "#, params!(description, equation))?;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<Equation>> {
        let mut results: HashMap<i64, i64> = HashMap::new();
        let mut query_stmt = self.conn.prepare("SELECT id FROM equations WHERE description LIKE ?")?;
        // Count number of hits for each entry
        for term in query.split_whitespace() {
            for id in query_stmt.query_map(params!(format!("%{}%", term)), |row| { Ok(row.get(0)?) })? {
                let id = id?;
                results.insert(id, results.get(&id).unwrap_or(&0) + 1);
            }
        }

        // Sort the results by number of hits and retrieve all corresponding entries
        let mut results: Vec<(i64, i64)> = results.iter().map(|(id, count)| (*id, *count)).collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(results.iter().map(|(id, _)| -> Equation {
            self.conn.query_row("SELECT description, equation FROM equations WHERE id = ?", params!(id), |row| {
                Ok(Equation {
                    description: row.get(0)?,
                    equation: row.get(1)?,
                })
            }).unwrap()
        }).collect())
    }

}

impl Equation {
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn html_equation(&self, display: bool) -> String {
        let opts = katex::Opts::builder()
            .display_mode(display)
            .leqno(false)
            .throw_on_error(false)
            .build().unwrap();
        katex::render_with_opts(&self.equation, opts).unwrap()
    }
}
