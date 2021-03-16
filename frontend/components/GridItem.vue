        <grid-item 
                   v-for="item in layout"
                   v-bind:key="item.i"
                   :static="item.static"
                   :x="item.x"
                   :y="item.y"
                   :w="item.w"
                   :h="item.h"
                   :i="item.i"
        >
            <div class="grid-item-container">
                <span class="text">{{itemTitle(item)}}</span>
                <div class="grid-item-main">{{inside}}</div>
            </div>
        </grid-item>