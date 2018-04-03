import Route from '@ember/routing/route';
import $ from 'jquery';
import { v4 } from 'ember-uuid'; 
import { inject as service } from '@ember/service';

export default Route.extend({
    currentDate: service('current-date'),
    admin: service('admin-priv'),
    beforeModel() {
        this.get('admin').adminRoute(this);
    },
    create_post_call: function(html_data) {
        $.ajax({
            type: "POST",
            crossDomain: true,
            contentType: "aplication/json; charset=utf-8",
            url: "http://localhost:8000/home_newspost",
            data: JSON.stringify({
                "username":username,
                "body":html_data,
                "datetime":this.get('currentDate').getDate(),
                "uuid":v4()
            }).done((/*response*/) => {

            }).fail((/*xhr*/) => {

            })
        });
    }
});
