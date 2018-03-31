import { inject } from '@ember/service';
import { run } from '@ember/runloop';
import $ from 'jquery';
import { Promise } from 'rsvp';
import OAuth2PasswordGrantAuthenticator from 'ember-simple-auth/authenticators/oauth2-password-grant';

export default OAuth2PasswordGrantAuthenticator.extend({
    session: inject('session'),
    authenticate: function(data) {
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