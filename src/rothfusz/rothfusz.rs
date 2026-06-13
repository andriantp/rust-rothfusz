
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Ideal,
    Humid,
    Caution,
    ExtremeCaution,
    Danger,
    ExtremeDanger,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Data {
    pub temp: f64,
    pub rh: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HeatIndexResult {
    pub heat_index_c: f64,
    pub category: Category,
    pub comfortable: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ResultData {
    pub data: Data,
    pub heat_index_result: HeatIndexResult,
}

pub trait RepositoryI {
    fn calculate_heat_index(
        &self,
        temp_c: f64,
        rh: f64,
    ) -> ResultData;
}

pub struct Repo {
    min_valid_temp_c: f64,
    humid_rh: f64,
}

impl Repo {
    pub fn new(
        min_valid_temp_c: f64,
        humid_rh: f64,
    ) -> Self {
        Self {
            min_valid_temp_c,
            humid_rh,
        }
    }

    fn calculate_rothfusz(&self,temp_c: f64,rh: f64) -> f64 {
        let t = self.c_to_f(temp_c);
        let hi =
            -42.379
            + 2.049_015_23 * t
            + 10.143_331_27 * rh
            - 0.224_755_41 * t * rh
            - 0.006_837_83 * t * t
            - 0.054_817_17 * rh * rh
            + 0.001_228_74 * t * t * rh
            + 0.000_852_82 * t * rh * rh
            - 0.000_001_99 * t * t * rh * rh;

        self.f_to_c(hi)
    }

    fn classify_heat_index(&self,hi_c: f64, rh: f64) -> Category {
        // Tropical humidity rule
        if hi_c < 27.0 && rh >= self.humid_rh {
            return Category::Humid;
        }

        match hi_c {

            x if x < 27.0 => Category::Ideal,

            x if x < 32.0 => Category::Caution,

            x if x < 41.0 => {
                Category::ExtremeCaution
            }

            x if x < 54.0 => Category::Danger,

            _ => Category::ExtremeDanger,
        }
    }

    fn c_to_f(&self,c: f64) -> f64 {
        (c * 9.0 / 5.0) + 32.0
    }

    fn f_to_c(&self,f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }
}

impl RepositoryI for Repo {
    fn calculate_heat_index(&self,temp_c: f64,rh: f64) -> ResultData {
        let hi_c =
            if temp_c < self.min_valid_temp_c {
                temp_c
            } else {
                self.calculate_rothfusz(temp_c, rh)
            };

        let category =
            self.classify_heat_index(hi_c, rh);

        ResultData {
            data: Data {
                temp: temp_c,
                rh,
            },

            heat_index_result: HeatIndexResult {
                heat_index_c: hi_c,
                comfortable:
                    category == Category::Ideal,
                category,
            },
        }
    }
}