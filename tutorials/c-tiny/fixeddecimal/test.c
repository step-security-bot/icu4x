// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XFixedDecimal.h"
#include "ICU4XFixedDecimalFormatter.h"
#include <string.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s <language>\n", argv[0]);
        return 1;
    }

    ICU4XLocale* locale = ICU4XLocale_create_und();
    if (!ICU4XLocale_set_language(locale, argv[1], strlen(argv[1])).is_ok) {
        printf("Invalid language tag \"%s\"\n", argv[1]);
        return 1;
    }

    ICU4XDataProvider* provider = ICU4XDataProvider_create_compiled();
    ICU4XFixedDecimal* decimal = ICU4XFixedDecimal_create_from_u64(1000007);
    ICU4XFixedDecimal_round(decimal, 0);

    ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result fdf_result =
        ICU4XFixedDecimalFormatter_create_with_grouping_strategy(provider, locale, ICU4XFixedDecimalGroupingStrategy_Auto);
    if (!fdf_result.is_ok)  {
        printf("Failed to create FixedDecimalFormatter\n");
        return 1;
    }
    ICU4XFixedDecimalFormatter* fdf = fdf_result.ok;
    char output[40];

    DiplomatWrite write = diplomat_simple_write(output, 40);

    ICU4XFixedDecimalFormatter_format(fdf, decimal, &write);
    if (write.grow_failed) {
        printf("format overflowed the string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    const char* expected = u8"১০,০০,০০৭";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    ICU4XFixedDecimal_destroy(decimal);
    ICU4XFixedDecimalFormatter_destroy(fdf);
    ICU4XLocale_destroy(locale);
    ICU4XDataProvider_destroy(provider);

    return 0;
}
