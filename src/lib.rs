// This file is part of try-finally. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/try-finally/master/COPYRIGHT. No part of try-finally, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of try-finally. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/try-finally/master/COPYRIGHT.


use std::panic::catch_unwind;
use std::panic::resume_unwind;
use std::panic::AssertUnwindSafe;


pub fn try_finally<F, R, C>(action: F, cleanup: C) -> R
where F: FnOnce() -> R, C: FnOnce(&std::thread::Result<R>) -> ()
{
	let result = catch_unwind(AssertUnwindSafe(||
	{
		action()
	}));
	
	cleanup(&result);
	
	match result
	{
		Err(panic) => resume_unwind(panic),
		Ok(result) => result
	}
}
