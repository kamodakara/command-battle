use serde::{Deserialize, Serialize};

// JSONエクスポート用レコード（idとdataを保持）
#[derive(Serialize, Deserialize)]
struct ExportRecord<T> {
    id: u32,
    data: T,
}

// 汎用リポジトリ
pub struct Record<T> {
    pub id: u32,
    pub data: T,
}

pub struct Repository<T> {
    records: Vec<Record<T>>,
    next_id: u32,
}

impl<T> Default for Repository<T> {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            next_id: 1,
        }
    }
}

impl<T: Clone> Repository<T> {
    /// データを登録し、登録されたレコードへの参照を返す
    pub fn create(&mut self, data: T) -> &Record<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.records.push(Record { id, data });
        self.records.last().unwrap()
    }

    /// 条件に一致する全レコードを返す
    pub fn find_many(&self, filter: impl Fn(&Record<T>) -> bool) -> Vec<&Record<T>> {
        self.records.iter().filter(|r| filter(r)).collect()
    }

    /// 全レコードを返す
    pub fn find_all(&self) -> Vec<&Record<T>> {
        self.records.iter().collect()
    }

    /// 条件に最初に一致するレコードを返す
    pub fn find_unique(&self, filter: impl Fn(&Record<T>) -> bool) -> Option<&Record<T>> {
        self.records.iter().find(|r| filter(r))
    }

    /// IDでレコードを取得する
    pub fn find_by_id(&self, id: u32) -> Option<&Record<T>> {
        self.records.iter().find(|r| r.id == id)
    }

    /// IDのレコードを更新し、更新後のレコードへの参照を返す
    pub fn update(&mut self, id: u32, mutate: impl Fn(&mut T)) -> Option<&Record<T>> {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == id) {
            mutate(&mut record.data);
            Some(record)
        } else {
            None
        }
    }

    /// IDのレコードを削除し、削除したレコードを返す
    pub fn delete(&mut self, id: u32) -> Option<Record<T>> {
        if let Some(pos) = self.records.iter().position(|r| r.id == id) {
            Some(self.records.remove(pos))
        } else {
            None
        }
    }

    /// 全レコードを削除する
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// 登録件数を返す
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// 全レコードをJSONファイルにエクスポートする
    /// idとdataを保持するため、インポート時に同じidで復元される
    pub fn export_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let export: Vec<ExportRecord<&T>> = self
            .records
            .iter()
            .map(|r| ExportRecord { id: r.id, data: &r.data })
            .collect();
        let json = serde_json::to_string_pretty(&export)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// JSONファイルからインポートする
    /// 現在のデータを全削除してからインポートする
    pub fn import_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let json = std::fs::read_to_string(path)?;
        let export: Vec<ExportRecord<T>> = serde_json::from_str(&json)?;
        self.records.clear();
        let max_id = export.iter().map(|r| r.id).max().unwrap_or(0);
        for record in export {
            self.records.push(Record { id: record.id, data: record.data });
        }
        self.next_id = max_id + 1;
        Ok(())
    }
}
