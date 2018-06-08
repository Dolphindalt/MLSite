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
            
            var promise = new Promise(function(resolve, reject) {
                $.ajax({
                    type: "POST",
                    url: "http://127.0.0.1:8000/register/" + comp.get('data.linkUuid'),
                    dataType: 'json',
                    contentType: "application/json; charset=utf-8",
                    crossDomain: true,
                    data: JSON.stringify({
                        "email":"", // let the backend deal with this when it validates the url
                        "hashword":hashword,
                        "admin":false,
                        "date_created":comp.get('currentDate').getDate(),
                        "uuid":comp.get('data.uuid'),
                        "staff":false,
                        "rank":"Default"
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
        }
    }
});
