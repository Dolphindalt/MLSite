import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    create_post_call: function(html_data) {
        $.ajax({
            type: "POST",
            crossDomain: true,
            contentType: "aplication/json; charset=utf-8",
            url: "http://localhost:8000/home_newspost",
            data: JSON.stringify({
                "username":username,
                "body":html_data
            }).done((response) => {

            }).fail((xhr) => {

            })
        });
    }
});
