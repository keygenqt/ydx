export const AppLog = {
    error: function (error) {
        console.error(error);
    },
    warn: function (error) {
        console.warn(error);
    },
    debug: function (data) {
        console.debug(data);
    },
    log: function (data) {
        if (typeof data === 'string' || data instanceof String) {
            console.log(data);
        } else {
            if (data.code && data.value && data.index !== undefined) {
                console.log(`${data.index} ${data.value}`);
            }
            else if (data.code && data.message) {
                if (data.value && data.value.stdout) {
                    console.log(`Code: ${data.code}, ${data.message}\n${data.value.stdout}`);
                } else {
                    console.log(`Code: ${data.code}, ${data.message}`);
                }
            } else {
                console.log(data);
            }
        }
    },
}
