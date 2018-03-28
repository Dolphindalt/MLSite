import OAuth2PasswordGrantAuthenticator from 'ember-simple-auth/authenticators/oauth2-password-grant';
import { inject } from '@ember/service';
import { run } from '@ember/runloop';
import $ from 'jquery';

export default OAuth2PasswordGrantAuthenticator.extend({
    session: inject('session'),
    authenticate: function(username, hashword) {
        return new Promise(function(resolve, reject) {
            $.ajax({
                type: "POST",
                contentType: "aplication/json; charset=utf-8",
                url: "http://localhost:8000/login",
                crossDomain: true,
                data: JSON.stringify({
                    "username":username,
                    "hashword":hashword
                })
            }).done((response) => {
                var token = JSON.parse(response);
                run(() => {
                    this.get('session').set('data.token', token.token);
                    resolve(token);
                });
            }).fail((xhr) => {
                var response = xhr.responseText;

                run(() => {
                    reject(response);
                });
            });
        });
    }
});
