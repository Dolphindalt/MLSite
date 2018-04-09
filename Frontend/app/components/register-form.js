import Component from '@ember/component';
import { inject as service } from '@ember/service';
import $ from 'jquery';
import SHA256 from 'cryptojs/sha256';

export default Component.extend({
    currentDate: service('current-date'),
    errorMessage: "",
    router: service(),
    actions: {
        success() {
            this.get('router').transitionTo('index');
        },
        register() {
            let { username, passwd, passwd2} = this.getProperties('username', 'passwd', 'passwd2');

            if(passwd != passwd2) {
                this.set('errorMessage', "The two passwords entered did not match");
                return;
            }

            if(username.length < 4) {
                this.set('errorMessage', "The username specified must be at least 4 characters");
                return;
            }

            if(passwd.length < 6) {
                this.set('errorMessage', "The password must be at least 6 characters");
                return;
            }
            
            var hashword = SHA256(passwd).toString();
            var comp = this; // stupid ajax
            
            $.ajax({
                type: "POST",
                url: "http://127.0.0.1:8000/register",
                dataType: 'json',
                contentType: "application/json; charset=utf-8",
                crossDomain: true,
                data: JSON.stringify({
                    "username":username,
                    "hashword":hashword,
                    "date_created":this.get('currentDate').getDate()
                }),
                error: function(xhr) {
                    comp.set('errorMessage', xhr.responseText);
                },
                success: function() {
                    comp.send('success');
                }
            });
        }
    }
});
