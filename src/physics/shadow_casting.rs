//! Shadow Casting
//! 재귀반복을 이용해 Symmetric Shadowcasting 을 만든다.
//!
//! psuedo 코드는 아래와 같다.
//!
//!```
//!  Scan(depth, startslope, endslope)
//!
//!    init y
//!    init x
//!
//!    while current_slope has not reached endslope do
//!      if (x,y) within visual range then
//!        if (x,y) blocked and prior not blocked then
//!          Scan(depth + 1, startslope, new_endslope)
//!        if (x,y) not blocked and prior blocked then
//!          new_startslope
//!        set (x,y) visible
//!      progress (x,y)
//!
//!    regress (x,y)
//!
//!    if depth < visual range and (x,y) not blocked
//!      Scan(depth + 1, startslope, endslope)
//!  end
//!```
