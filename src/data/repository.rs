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
}
