import Component from '@ember/component';
import { inject as service } from '@ember/service';
import $ from 'jquery';
import SHA256 from 'cryptojs/sha256';
import { run } from '@ember/runloop';
import { Promise } from 'rsvp';

export default Component.extend({
    classNames: ['sign-in-form'],
    currentDate: service('current-date'),
    uuid: service('uuid-to-username'),
    errorMessage: "",
    registering: true,
    router: service(),
    session: service('auth-service'),
    willRender() {
        if(this.get('session.isAuthenticated') === true)
            this.get('router').transitionTo('index');
    },
    actions: {
        success() {
            this.set('registering', false);
        },
        register() {
            let { passwd, passwd2} = this.getProperties('passwd', 'passwd2');

            if(passwd != passwd2) {
                this.set('errorMessage', "The two passwords entered did not match");
                return;
            }

            if(passwd.length < 6) {
                this.set('errorMessage', "The password must be at least 6 characters");
                return;
            }

            var hashword = SHA256(passwd).toString();
            var comp = this;
            
            comp.get('uuid').uuidToUsername(comp.get('data.uuid')).then((payload) => {
                var username = payload;

                var promise = new Promise(function(resolve, reject) {
                    $.ajax({
                        type: "POST",
                        url: "http://127.0.0.1:8000/register/" + comp.get('data.linkUuid'),
                        dataType: 'json',
                        contentType: "application/json; charset=utf-8",
                        crossDomain: true,
                        data: JSON.stringify({
                            "email":comp.get('data.email'),
                            "hashword": hashword,
                            "username": username,
                            "admin": false,
                            "date_created": comp.get('currentDate').getDate(),
                            "uuid": comp.get('data.uuid'),
                            "staff":false,
                            "rank":"Default",
                            "banned":false,
                        })
                    }).done(() => {
                        run(() => {
                            resolve();
                        });
                    }).fail((xhr) => {
                        reject(xhr.responseText);
                    });
                });
    
                promise.then(() => {
                    comp.send('success');
                }).catch((stuff) => {
                    comp.set('errorMessage', stuff);
                });
            }).catch(() => {
                this.set('errorMessage', "Failed to resolve username to uuid!");
                return;
            });
        }
    }
});
