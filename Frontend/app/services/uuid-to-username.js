import Service from '@ember/service';
import { Promise } from 'rsvp';
import $ from 'jquery';

export default Service.extend({
    uuidToUsername(uuid) {
        let uuid_trim = uuid.replace(/-|\s/g,"", "");
        return new Promise(function(resolve, reject) {
            $.ajax({
                type: "GET",
                dataType: 'json',
                url: "https://api.minetools.eu/uuid/" + uuid_trim,
                success: (data) => {
                    resolve(data.name);
                },
                error: () => {
                    reject(false);
                }
            });
        });
    },
    usernameToUuid(username) {
        return new Promise(function(resolve, reject) {
            $.ajax({
                type: "GET",
                dataType: 'json',
                url: "https://api.minetools.eu/uuid/" + username,
                success: (data) => {
                    resolve(data.id);
                },
                error: () => {
                    reject(false);
                }
            });
        });
    }
});
