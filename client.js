const { Retcher, Browser } = require('./index');

(async () => {
    const client = new Retcher({ browser: Browser.Firefox, ignoreTlsErrors: true });

    try {
        console.log('Fetching...');
        const x = await client.retch('https://httpbin.org/absolute-redirect/4');
        console.log(x.body.toString());
    } catch (e) {
        console.log(e.message);
    }
})();