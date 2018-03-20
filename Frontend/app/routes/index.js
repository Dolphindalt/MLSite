import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model() {
        //return $.getJSON("127.0.0.1:8000/home");
        return $.ajax({
            type: "GET",
            dataType: 'jsonp',
            url: "127.0.0.1:8000/home",
            async: false,
            contentType: "application/json; charset=utf-8"
        });
    }
});
