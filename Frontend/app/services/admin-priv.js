import Service from '@ember/service';
import $ from 'jquery';
import { run } from '@ember/runloop';

export default Service.extend({
    isAdmin() {
        return new Promise((resolve, reject) => {
            $.ajax({
                type: "GET",
                url: "http://127.0.0.1:8000/admin",
            }).always((data, status, xhr) => {
                if(xhr.status !== 200) {
                    run(() => {
                        reject(false);
                    });
                } else {
                    run(() => {
                        resolve(true);
                    });
                }
            });
        });
    }
});
