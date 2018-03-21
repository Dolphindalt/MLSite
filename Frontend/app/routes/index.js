import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model() {
        //return $.getJSON("https://www.reddit.com/r/anime/.json");
        return $.ajax({
            type: "GET",
            dataType: 'json',
            url: "http://127.0.0.1:8000/home",
            crossDomain: true,
            contentType: "application/json; charset=utf-8",
            error: function(xhr, err) {
                console.debug(xhr);
                console.debug(err);
            }
        });
    }
});
