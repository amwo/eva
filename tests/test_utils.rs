extern crate eva;
use eva::utils::*;

#[test]
fn test_comp() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en-US" prefix="og: http://ogp.me/ns#">
            <head>
                <meta charset="utf-8" />
                <link rel="canonical" href="domain" />
                <link rel="alternate" href="domain" hreflang="en-US" />
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
                <link rel="stylesheet" type="text/css" href="/css/main.css" />
                <title>title</title>
                <meta name="Description" content="description" />
                <script src="https://images.apple.com/metrics/ac-analytics/2.2/scripts/gpu.js" type="text/javascript" charset="utf-8"></script>
                <meta property="og:title" content=""title />
                <meta property="og:description" content="description" />
                <meta property="og:url" content="domain" />
                <meta property="og:locale" content="en-US" />
                <meta property="og:image" content="" />
                <meta property="og:type" content="website" />
                <meta property="og:site_name" content="title" />
            </head>
            <header>
                <h1>Hello world!</h1>
            </header>
            <footer>
                <input type="text" value="" placeholder="" />
            </footer>
            <body>
                <script type="application/ld+json">
                    {
                    "@context": "http://schema.org",
                    "@id": "domain/#webpage",
                    "@type": "WebPage",
                    "url": "domain",
                    "name": "title"
                }
                </script>
                <script>
                    console.log('hello!')
                    var test;
                    test = 'hoge';
                    fire = 'fire'
                    console.log(test + fire)
                </script>
                <script src="/js/main.js" type="text/javascript"></script>
            </body>
        </html>
    "#;
    println!("{}", "===============================================================================");
    comp(html);
    //println!("{:#?}", comp(html));
    println!("{}", "===============================================================================");
}
