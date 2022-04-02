impl FurTable {
    pub fn generate_sortfile(&self) {
        let rows = self.get_row_bin();
        let columns = self.table_info.get_columns();
    }
}
