use serde::{Deserialize, Serialize};
const DEFAULT_PAGE_SIZE: u64 = 15;
const ONE_SIDE_PAGE_SIZE: u64 = 3;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Pagination {
    /** 总记录数 */
    pub item_count: u64,
    /** 每页记录数 */
    pub page_size: u64,
    /** 总页数 */
    pub page_count: u64,
    /** 当前页 */
    pub page_no: u64,
    /** 当前页的记录数 */
    pub item_size: u64,
    /** 页是否越界 */
    pub out_of_bounds: bool,
    /** 是否有上一页 */
    pub has_pre_page: bool,
    /** 是否有下一页 */
    pub has_next_page: bool,
    /** 单边需要显示页数 */
    pub one_side_page_count: u64,
    /** 当前页第一条记录的索引 */
    pub start_item: u64,
    /** 当前页最后一条记录的索引 */
    pub end_item: u64,
    /** 最左边页 */
    pub start_page: u64,
    /** 最右边页 */
    pub end_page: u64,
}
impl Pagination {
    pub fn new(
        item_count: u64,
        page_no: u64,
        page_size: Option<u64>,
        one_side_page_count: Option<u64>,
    ) -> Pagination {
        let mut inst = Pagination {
            item_count: item_count,
            page_size: DEFAULT_PAGE_SIZE,
            page_count: 0,
            page_no: page_no,
            item_size: 0,
            out_of_bounds: false,
            has_pre_page: false,
            has_next_page: false,
            one_side_page_count: ONE_SIDE_PAGE_SIZE,
            start_item: 0,
            end_item: 0,
            start_page: 0,
            end_page: 0,
        };
        match page_size {
            Some(v) => {
                inst.page_size = v;
            }
            _ => (),
        }
        match one_side_page_count {
            Some(v) => {
                inst.one_side_page_count = v;
            }
            _ => (),
        }
        inst.calculate_page_count(); //计算页数
        inst.modify_page_no(); //修正当前页
        inst.calculate_other(); //计算其它
        return inst;
    }

    fn calculate_other(&mut self) {
        if 0 == self.page_count {
            self.has_pre_page = false;
            self.has_next_page = false;
            self.item_size = 0;
            self.start_item = 0;
            self.end_item = 0;
            self.start_page = 0;
            self.end_page = 0;
        } else {
            if 1 >= self.page_no {
                self.has_pre_page = false;
            } else {
                self.has_pre_page = true;
            }
            if self.page_count <= self.page_no {
                self.has_next_page = false;
            } else {
                self.has_next_page = true;
            }
            if self.page_no == self.page_count {
                self.item_size = self.item_count - self.page_size * (self.page_count - 1);
            } else {
                self.item_size = self.page_size;
            }
            self.start_item = self.page_size * (self.page_no - 1) + 1;
            self.end_item = self.page_size * (self.page_no - 1) + self.item_size;

            let start_page: u64;
            let end_page: u64;
            if self.page_count <= 2 * self.one_side_page_count + 1 {
                //总页数小于等于需要显示的页数
                start_page = 1;
                end_page = self.page_count;
            } else {
                if self.page_no <= self.one_side_page_count + 1 {
                    start_page = 1;
                    end_page = 2 * self.one_side_page_count + 1;
                } else if self.page_no >= self.page_count - self.one_side_page_count {
                    start_page = self.page_count - 2 * self.one_side_page_count;
                    end_page = self.page_count;
                } else {
                    start_page = self.page_no - self.one_side_page_count;
                    end_page = self.page_no + self.one_side_page_count;
                }
            }
            self.start_page = start_page;
            self.end_page = end_page;
        }
    }

    fn calculate_page_count(&mut self) {
        if 0 == self.item_count % self.page_size {
            self.page_count = self.item_count / self.page_size;
        } else {
            self.page_count = self.item_count / self.page_size + 1;
        }
    }

    fn modify_page_no(&mut self) {
        self.out_of_bounds = false;
        if self.page_count < self.page_no {
            self.page_no = self.page_count;
            self.out_of_bounds = true;
        }
        if 1 > self.page_no {
            self.page_no = 1;
            self.out_of_bounds = true;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationList<T> {
    pub pagination: Pagination,
    pub list: Vec<T>,
}
