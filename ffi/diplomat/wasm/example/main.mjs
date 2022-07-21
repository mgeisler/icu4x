import { ICU4XFixedDecimal, ICU4XDataProvider, ICU4XLocale, ICU4XFixedDecimalFormatter, ICU4XFixedDecimalGroupingStrategy } from "../lib/api.mjs"

const decimal = ICU4XFixedDecimal.create(1234);
decimal.multiply_pow10(-2);
decimal.set_sign("Negative");
console.log(decimal.to_string());

const dataProvider = ICU4XDataProvider.create_test().provider;

const locale = ICU4XLocale.create("bn");

const format = ICU4XFixedDecimalFormatter.try_new(locale, dataProvider, "Auto").fdf;
console.log(format.format(decimal));