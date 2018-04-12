import Component from '@ember/component';
import { inject as service } from '@ember/service';
import $ from 'jquery';

export default Component.extend({
    math: service('math'),
    classNames: ['home-canvas'],
    tagName: 'canvas',
    width: 600,
    height: 300,
    //particles: [],
    particlesNum: 70,
    //colors: ["#111111", "#6d0000"],
    attributeBindings: ['width', 'height'],
    mycanvas: undefined,
    ctx: undefined,
    didRender() {
        this.particles = [];
        this.colors = ["#111111", "#6d0000"];
        this.attributeBindings = ['width', 'height'];

        this.set('mycanvas', $("canvas")[0]);
        this.set('ctx', this.get('mycanvas').getContext("2d"));
        
        for(let i = 0; i < this.get('particlesNum'); i++) {
            this.get('particles').push(this.makeParticle());
        }

        this.send('activateParticles');
    },
    draw() {
        let ctx = this.get('ctx');
        ctx.clearRect(0, 0, this.get('width'), this.get('height'));
        for(let i = 0; i < this.get('particlesNum'); i++) {
            let temp = this.get('particles')[i];
            let factor = 1;
            for(let j = 0; j < this.get('particlesNum'); j++) {
                let temp2 = this.get('particles')[j];
                ctx.linewidth = 0.5;

                let distance = this.get('math').distance(temp, temp2);
                if(temp.rgba == temp2.rgba && distance < 50) {
                    ctx.strokeStyle = "#464646" + parseInt(Math.round(distance), 16);
                    ctx.beginPath();
                    ctx.moveTo(temp.x, temp.y);
                    ctx.lineTo(temp2.x, temp2.y);
                    ctx.stroke();
                    factor += 0.1;
                }
            }

            ctx.fillStyle = temp.rgba;
            ctx.strokeStyle = temp.rgba;

            ctx.beginPath();
            ctx.arc(temp.x, temp.y, temp.rad*factor, 0, Math.PI*2, true);
            ctx.fill();
            ctx.closePath();

            temp.x += temp.vx;
            temp.y += temp.vy;

            if(temp.x > this.get('width'))temp.x = 0;
            if(temp.x < 0)temp.x = this.get('width');
            if(temp.y > this.get('height'))temp.y = 0;
            if(temp.y < 0)temp.y = this.get('height'); 
        }
    },
    makeParticle() {
        var g = {
            "x": Math.round( Math.random() * this.get('width')),
            "y": Math.round( Math.random() * this.get('height')),
            "rad": Math.round( Math.random() * 1) + 1,
            "rgba": this.get('colors')[ Math.round( Math.random() * (this.get('colors').length - 1)) ],
            "vx": Math.round( Math.random() * 2) - 1.5,
            "vy": Math.round( Math.random() * 2) - 1.5
        };
        return g;
    },
    actions: {
        activateParticles() {
            let stuff = () => {
                this.draw();
            }
            setInterval(stuff, 60);
        }
    }
});
