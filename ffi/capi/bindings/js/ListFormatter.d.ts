import { FFIError } from "./diplomat-runtime"
import { ICU4XDataError } from "./ICU4XDataError";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XLocale } from "./ICU4XLocale";
import { ListLength } from "./ListLength";

/**

 * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html Rust documentation for `ListFormatter`} for more information.
 */
export class ListFormatter {

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_and_with_length Rust documentation for `try_new_and_with_length`} for more information.
   * @throws {@link FFIError}<{@link ICU4XDataError}>
   */
  static create_and_with_length(provider: ICU4XDataProvider, locale: ICU4XLocale, length: ListLength): ListFormatter | never;

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_or_with_length Rust documentation for `try_new_or_with_length`} for more information.
   * @throws {@link FFIError}<{@link ICU4XDataError}>
   */
  static create_or_with_length(provider: ICU4XDataProvider, locale: ICU4XLocale, length: ListLength): ListFormatter | never;

  /**

   * Construct a new ICU4XListFormatter instance for And patterns

   * See the {@link https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_unit_with_length Rust documentation for `try_new_unit_with_length`} for more information.
   * @throws {@link FFIError}<{@link ICU4XDataError}>
   */
  static create_unit_with_length(provider: ICU4XDataProvider, locale: ICU4XLocale, length: ListLength): ListFormatter | never;
}
