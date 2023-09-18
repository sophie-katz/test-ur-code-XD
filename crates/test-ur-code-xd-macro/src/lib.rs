// Copyright (c) 2023 Sophie Katz
//
// This file is part of test-ur-code-XD.
//
// test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
// not, see <https://www.gnu.org/licenses/>.

mod parameters;
mod permute;

use parameters::{get_paraneterizations, parameterize_test_function};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test_with_parameter_values(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let parameterizations = get_paraneterizations(attribute);
    let item = parse_macro_input!(item as ItemFn);

    TokenStream::from(parameterize_test_function(item, parameterizations))
}
