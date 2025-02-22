use rps_dns::error::*;
use rps_dns::query;
use rps_dns::resolver::*;

#[test]
fn test_query_a() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["94.140.14.140".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_a("www.baidu.com".to_string());
    if let Some(answer) = result.a() {
        println!("{}", answer);
    } else {
        println!("No A record");
        #[cfg(feature = "fmt")]
        println!("{}", result);
    }
}

#[test]
fn test_query_aaaa() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["94.140.14.140".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_aaaa("www.google.com".to_string());
    if let Some(answer) = result.aaaa() {
        println!("{}", answer);
    } else {
        println!("No AAAA record");
        #[cfg(feature = "fmt")]
        println!("{}", result);
    }
}

#[test]
fn test_query_cname() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["94.140.14.140".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_cname("www.baidu.com".to_string());
    if let Some(answer) = result.cname() {
        println!("{}", answer);
    } else {
        println!("No CNAME record");
        #[cfg(feature = "fmt")]
        println!("{}", result);
    }
}

#[test]
fn test_query_soa() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["94.140.14.140".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_soa("www.baidu.com".to_string());
    if let Some(answer) = result.soa() {
        #[cfg(feature = "fmt")]
        println!("{}", answer);
        #[cfg(not(feature = "fmt"))]
        println!("{:?}", result);
    } else {
        println!("No SOA record");
        #[cfg(feature = "fmt")]
        println!("{}", result);
    }
}

#[test]
fn test_query_txt() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["9.9.9.9".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_txt("fs.gloryouth.com".to_string());
    if let Some(answer) = result.txt() {
        #[cfg(feature = "fmt")]
        println!("{:?}", answer);
        #[cfg(not(feature = "fmt"))]
        println!("{:?}", result);
    } else {
        println!("No TXT record");
        #[cfg(feature = "fmt")]
        println!("{}", result);
    }
}

#[test]
#[cfg(feature = "fmt")]
fn test_fmt() {
    #[cfg(feature = "logger")]
    init_logger();
    #[cfg(feature = "logger")]
    set_println_enabled(true);
    let mut server = vec!["223.5.5.5".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_txt("fs.gloryouth.com".to_string());
    println!("{}", result);
}

#[test]
fn test_special() {
    #[cfg(feature = "logger")]
    init_logger();
    let mut server = vec!["9.9.9.9".to_string()];
    let resolver = Resolver::new(&mut server).unwrap();
    let result = resolver.query_txt("gloryouth.com".to_string());
    println!(
        "{:?}",
        result
            .txt_iter()
            .unwrap()
            .flatten()
            .collect::<Vec<String>>()
    );
}

#[test]
#[cfg(feature = "result_error")]
fn test_query() {
    let server = vec!["9.9.9.9".to_string()];
    let result = query! {
        a,
        @target "www.baidu.com".to_string(),
        @server server,
        -error
    };
    println!("{:?}", result.result().as_ref());
}
