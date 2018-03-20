import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model() {
        //return $.getJSON("https://www.reddit.com/r/anime/.json");
        //return $.getJSON("127.0.0.1:8000/home");
        return $.ajax({
            type: "GET",
            dataType: 'jsonp',
            url: "http://127.0.0.1:8000/home",
            async: false,
            crossDomain: true,
            contentType: "application/json; charset=utf-8",
            error: function(xhr, err) {
                console.debug(xhr);
                console.debug(err);
            },
            jsonpCallback: function(data) {
                return data;
            }
        });
    }
});
