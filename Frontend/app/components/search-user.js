import Component from '@ember/component';
import $ from 'jquery';

export default Component.extend({
    tagName: 'div',
    classNames: ['search-bar-wrapper'],
    didRender() {
        let searchbar = $("input")[0];
        let timeout = null;
        console.debug(searchbar);
        searchbar.onkeyup = function() {
            clearTimeout(timeout);

            timeout = setTimeout(() => {
                console.debug(searchbar.value);
            }, 1000);
        }
    },
    actions: {
        search() {

        }
    }
});
