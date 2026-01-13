use rusqlite::{Connection, Error, OptionalExtension as _, Result as RusqliteResult, params};
use std::sync::Mutex; // 👈 adicione isto
use crate::domain::simulado::Simulado;

pub struct SimuladoRepository {
    conn: Mutex<Connection>, // 👈 envolva com Mutex
}

impl SimuladoRepository {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn salvar(&self, simulado: &Simulado) -> RusqliteResult<i64> {
        let conn = self.conn.lock().unwrap(); // 👈 trava a conexão
        
        let estado = simulado.estado()
            .map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        let estado_json = serde_json::to_string(&estado)
            .map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;

        if simulado.id == 0 {
            conn.execute(
                "INSERT INTO simulado (
                    vestibular, ano, prova_id, tempo_limite,
                    iniciado_em, finalizado_em, estado_json
                ) VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    &simulado.vestibular,
                    simulado.ano,
                    &simulado.prova_id,
                    simulado.tempo_limite,
                    &simulado.iniciado_em,
                    &simulado.finalizado_em,
                    &estado_json,
                ],
            )?;
            Ok(conn.last_insert_rowid())
        } else {
            conn.execute(
                "UPDATE simulado SET
                    vestibular = ?, ano = ?, prova_id = ?, tempo_limite = ?,
                    iniciado_em = ?, finalizado_em = ?, estado_json = ?
                 WHERE id = ?",
                params![
                    &simulado.vestibular,
                    simulado.ano,
                    &simulado.prova_id,
                    simulado.tempo_limite,
                    &simulado.iniciado_em,
                    &simulado.finalizado_em,
                    &estado_json,
                    simulado.id,
                ],
            )?;
            Ok(simulado.id)
        }
    }

    pub fn buscar_por_id(&self, id: i64) -> RusqliteResult<Option<Simulado>> {
        let conn = self.conn.lock().unwrap();
        conn
            .query_row(
                "SELECT * FROM simulado WHERE id = ?",
                [id],
                Simulado::from_row,
            )
            .optional()
    }

    pub fn listar_por_vestibular(&self, vestibular: &str) -> RusqliteResult<Vec<Simulado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM simulado
             WHERE vestibular = ?
             ORDER BY iniciado_em DESC"
        )?;

        let simulados = stmt
            .query_map([vestibular], Simulado::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(simulados)
    }

    pub fn listar_todos(&self) -> RusqliteResult<Vec<Simulado>> {
    let conn = self.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM simulado ORDER BY iniciado_em DESC")?;
    let simulados = stmt
        .query_map([], Simulado::from_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(simulados)
}
}