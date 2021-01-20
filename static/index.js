Vue.component('party-member', {
    props: ['actor'],
    template: '<div class="actor"><img v-if="actor.name" :src=imgSrc(actor.name)></div>',
    methods: {
        imgSrc(name) {
            return `http://www.videogamesprites.net/FinalFantasy6/Party/${name}/${name}%20-%20Walk%20(Left).gif`;
        }
    }
})

var app = new Vue({
    el: '#app',
    data: {
        party: [
            { id: -1, name: "" },
            { id: -2, name: "" },
            { id: -3, name: "" },
            { id: -4, name: "" },
        ],
        polling: null,
    },
    methods: {
        loadParty() {
            fetch('/party')
                .then(response => response.json())
                .then(data => (this.party = data));
        },
    },
    mounted: function () {
        console.log("ready");
        this.loadParty();
        this.polling = setInterval(() => this.loadParty(), 1000);
    },
})