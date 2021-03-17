fn main() {
    // Rust does not have exceptions, instead functions that can fail have a return type that says so
    // fn get_weater(location: LatLang) -> Result<WeatherReport, io::Error>
    // This function will either return a success result Ok(weather) where weather is a new WeatherReport value
    // or an error result Err(error_value), where error_value is an io::Error explaining what went wrong

    // Rust requires us to write some kind of error handling for these types of function

    // CATCHING ERRORS

    // match get_weather(hometown) {
    //     Ok(report) => {
    //         display_weather(hometown, &report);
    //     }
    //     Err(err) => {
    //         println!("Error querying the weather: {}", err);
    //         schedule_weather_entry();
    //     }
    // }

    // Common methods on result:-

    // 1. result.is_ok() and result.is_err() return bool telling if result is a success result or error

    // 2. result.ok() returns the success value, if any, as an Option<T>
    // If the result is a success result, this returns Some(success_value) otherwise None, discarding the error value

    // 3. result.err() returns the error value, if any, as Option<E>

    // 4. result.unwrap_or(fallback) returns the success value, if result is a success train
    // Otherwise it returns fallback, discarding the error value

    // const THE_USUAL: WeatherReport = WeatherReport::Sunny(72);
    // let report = get_weather(loss_angeles).unwrap_or(THE_USUAL);
    // display_weather(los_angeles, &report)

    // This only works when there is an appropriate fallback value

    // 5. result.unwrap_or_else(fallback_fn) is the same, but instead of passing a fallback value
    // directly, we pass a function or closure
    // This is for cases when it is wasteful to compute the fallback value if we are not going to use it
    // The fallback_fn is called only if we have an error results

    // let report = get_weather(hometown).unwrap_or_else(|_err| vague_prediction(hometown));

    // 6. result.unwrap() also returns the success value, if result is a success result
    // However, if result is an error result, this method panics

    // 7. reuslt.expect(message) is same as .unwrap() but lets us provide a message that it prints in case of panic

    // 8. result.as_ref() converts a Result<T, E> to a Result<&T, &E> borrowing a reference to the success or 
    // error value in existing result

    // 9. result.as_mut() is the same, but borrows a mutable reference, the return type is Result<&mut T, &mut E>

    // Except 1, 8 and 9 all other consume the result they operate on

    // RESULT TYPE ALIAS

    // fn remove_file(path: &Path) -> Result<()>
    // pub type Result<T> = result::Result<T, Error>
    // This is defined in std::io
    // Rust will then understand Result<String> as Result<String, io::Error>

    // PRINTING ERRORS

    // 1. println!()
    // 2. err.description() returns an error message as a &str
    // 3. err.cause() returns an Option<&Error> - the underlying error if any that triggered err

    // This function can be used to print all error

    use std::error::Error;
    use std::io::{Write, stderr};

    fn _print_error(mut err: &dyn Error) { // We have to do &dyn now instead of &Error
        let _ = writeln!(stderr(), "Error: {}", err);
        while let Some(cause) = err.source() { // cause has changed to source
            let _ = writeln!(stderr(), "Caused by: {}", cause);
            err = cause;
        }
    }

    // PROPAGATING ERRORS

    // let weather = get_weather(hometown)?;
    // This will propagate the error up the call stack and the caller will handle this error
    // On success it unwraps the Result to get the success value inside
    // On error it immediately returns from the enclosing function passing the error result up the call chain
    // ? can only be used with functions that have Result return type

    use std::fs;
    use std::io;
    use std::path::Path;

    fn _movel_all(src: &Path, dst: &Path) -> io::Result<()> {
        for entry_result in src.read_dir()? {
            let entry = entry_result?;
            let dst_file = dst.join(entry.file_name());
            fs::rename(entry.path(), dst_file)?;
        }

        Ok(())
    }

    // WORKING WITH MULTIPLE ERROR TYPES

    // use std::io::{BufRead};

    // fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    //     let mut numbers = vec![];
    //     for line_result in file.lines() {
    //         let line = line_result?; // reading lines can fail
    //         numbers.push(line.parse()?); // parsing integers can fail
    //     }

    //     Ok(numbers)
    // }

    // Parsing failure returns std::num::ParseIntError which cannot be converted to io::Error
    // To solve this we can use Box<std::error::Error> which represents "any error"

    // type GenError = Box<std::error::Error>;
    // type GenResult<T> = Result<T, GenError>;

    // Then return type of the read_numbers function can be made GenResult<Vec<i64>>
    
    // ERROR THAT CAN'T HAPPEN

    // If you are certain some error won't happen then we can just return ? from the function
    // and in the calling function use .unwrap() method

    // IGNORING ERRORS

    // use _ to catch that:- let _ = writeln!(stderr(), "Error: {}", err);

    // HANDLING ERRORS IN main()

    // In main we cannot use ?
    // The simplest solution to use here is .expect(message)

    // Another way is to handle it:-
    // if let Err(err) = calculate_tides() {
    //     print_erro(s&err);
    //     std::process::exit(1);
    // }
}
