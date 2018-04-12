import Service from '@ember/service';

export default Service.extend({
    uuidToUsername(uuid) {
        return new Promise((resolve, reject) => {
            $.getJSON("https://sessionserver.mojang.com/session/minecraft/profile/" + uuid).then((data) => {
                resolve(data.name);
            });
        });
        reject(false);
    }
});
