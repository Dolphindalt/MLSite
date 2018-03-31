import Service from '@ember/service';
import { run } from '@ember/runloop';
import $ from 'jquery';
import { Promise } from 'rsvp';

export default Service.extend({
    token: null,
    isAuthenticated: false,
    invalidate: function() {
        this.set('isAuthenticated', false);
        this.set('token', null);
    },
    authenticate: function(data) {
        let comp = this;
        return new Promise(function(resolve, reject) {
            $.ajax({
                type: "POST",
                crossDomain: true,
                url: 'http://127.0.0.1:8000/login',
                contentType: "application/json; charset=utf-8",
                data: JSON.stringify({
                    username: data.username,
                    password: data.hashword
                })
            }).done((res) => {
                run(() => {
                    comp.set('isAuthenticated', true);
                    resolve({token:res.token});
                });
            }).fail((xhr) => {
                let res = xhr.responseText;
                run(() => {
                    reject(res);
                });
            });
        });
    }
});
