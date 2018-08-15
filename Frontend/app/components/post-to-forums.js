import Component from '@ember/component';
import { v4 } from 'ember-uuid';
import $ from 'jquery';
import { inject as service } from '@ember/service';
import { run } from '@ember/runloop';

export default Component.extend({
    tagName: '',
    errorMessage: '',
    session: service('auth-service'),
    date: service('current-date'),
    router: service(),
    actions: {
        postToCategory(cat_name) {
            let thread_title = this.get("thread_title");
            let thread_body = this.get("thread_body");
            let author_uuid = this.get("session").get("uuid");
            let author_username = this.get("session").get("username");
            let post_uuid = v4();
            let chain_uuid = v4();
            let date = this.get('date').getDate();

            run(() => {
                $.ajax({
                    type: "POST",
                    url: "http://127.0.0.1:8000/forums/newthread/" + cat_name,
                    crossDomain: true,
                    contentType: "application/json; charset=utf-8",
                    data: JSON.stringify({
                        "chain_uuid":chain_uuid,
                        "posts":[
                            { 
                                "title":thread_title,
                                "body":thread_body,
                                "author": author_username,
                                "datetime":date,
                                "uuid":post_uuid,
                                "author_uuid":author_uuid
                            }
                        ]
                    })
                });
            });

            this.sendAction('transitionToThread', 'home.forum-thread', cat_name, chain_uuid, 1);
        },
        postToThread(cat_name, thread_uuid) {
            let post_body = this.get("post_body");
            let author_uuid = this.get("session").get("uuid");
            let author_username = this.get("session").get("username");
            let post_uuid = v4();
            let date = this.get('date').getDate();

            run(() => {
                $.ajax({
                    type: "POST",
                    url: "http://127.0.0.1:8000/forums/newpost/" + cat_name + "/" + thread_uuid,
                    crossDomain: true,
                    contentType: "application/json; charset=utf-8",
                    data: JSON.stringify({
                            "title":"",
                            "body":post_body,
                            "author": author_username,
                            "datetime":date,
                            "uuid":post_uuid,
                            "author_uuid":author_uuid
                    })
                });
            });

            this.sendAction('transitionToThread', 'home.forum-thread');
        }
    }
});
