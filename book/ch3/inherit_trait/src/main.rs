trait Page {
    fn set_page(&self, _p: i32) {
        println!("Page Default: 1");
    }
}

trait PerPage {
    fn set_per_page(&self, _num: i32) {
        println!("PerPage Default: 10");
    }
}

struct MyPaginate { page: i32 }
impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

fn test_mypaginate() {
    let my_paginate = MyPaginate{page: 1};
    my_paginate.set_page(2);
    my_paginate.set_per_page(100);
}

// Inheritance Page and PerPage
trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Default: {:?}", num);
    }
}
impl <T: Page + PerPage> Paginate for T {}

fn test_inherit_trait() {
    let my_paginate = MyPaginate{page: 1};
    my_paginate.set_page(1);
    my_paginate.set_per_page(100);
    my_paginate.set_skip_page(12);
}

fn main() {
    test_mypaginate();
    test_inherit_trait();
}
