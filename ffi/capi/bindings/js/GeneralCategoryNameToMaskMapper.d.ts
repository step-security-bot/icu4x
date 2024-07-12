import { u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * A type capable of looking up General Category mask values from a string name.

 * See the {@link https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.name_to_enum_mapper Rust documentation for `name_to_enum_mapper`} for more information.

 * See the {@link https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html Rust documentation for `PropertyValueNameToEnumMapper`} for more information.
 */
export class GeneralCategoryNameToMaskMapper {

  /**

   * Get the mask value matching the given name, using strict matching

   * Returns 0 if the name is unknown for this property
   */
  get_strict(name: string): u32;

  /**

   * Get the mask value matching the given name, using loose matching

   * Returns 0 if the name is unknown for this property
   */
  get_loose(name: string): u32;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.name_to_enum_mapper Rust documentation for `name_to_enum_mapper`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static load(provider: DataProvider): GeneralCategoryNameToMaskMapper | never;
}