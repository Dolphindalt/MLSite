import Component from '@ember/component';
import { inject as service } from '@ember/service';

export default Component.extend({
    tagName: '',
    date: service('current-date'),
    willRender() {
        if(!this.get('data'))
            return;
        for(let i = 0; i < this.get("data").length; i++) {
            let num = Number(this.get("data")[i].len);
            if(num === 1) {
                this.set('data.' + i + '.word', "Reply");        
            } else {
                this.set('data.' + i + '.word', "Replies");
            }
            let datetime = this.get('data')[i].datetime;
            let how_long = this.get('date').getHowMuchTimeAgo(datetime);
            this.set('data.' + i + '.timeago', how_long);
        }
        
    }
});
