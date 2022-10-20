pub fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut maximum_height_candle = i32::MIN;
    let mut times =0;
    for height_of in candles {
        if height_of > &maximum_height_candle {
            maximum_height_candle = *height_of;
            times = 1;
        }else if height_of == &maximum_height_candle{
            times+=1;
        }
    }
    times
}