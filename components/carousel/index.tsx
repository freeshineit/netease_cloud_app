import React, { useState, FC, useMemo } from "react";
import cls from "classnames";
import styled from "styled-components";

import { useCallback } from "react";

interface CarouselProps extends Omit<NetEase.CommonComponentProps, "children"> {
  list: any[];
}

const CarouselCom = styled.div`
  height: 200px;
  position: relative;
  background-color: transparent;

  .item-wrapper.active_prev .item-content,
  .item-wrapper.active_next .item-content {
    position: relative;
    height: 200px;
    width: 540px;
  }

  .prev,
  .next {
    position: absolute;
    z-index: 10;
    top: 0;
    bottom: 0;
    width: 100px;
    font-size: 20px;
    color: #fff;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .prev {
    left: 0;
  }

  .next {
    right: 0;
  }
`;

const Item = styled.div`
  height: 200px;
  width: 540px;
  position: relative;
  border-radius: 8px;
  color: #fff;

  img {
    height: 200px;
    width: 540px;
    outline: none;
    font-size: 0;
  }
`;

const CarouselItemWrapper = styled.div`
  height: 200px;
  position: absolute;
  text-align: center;
  width: 100%;
  display: flex;
  justify-content: center;
  transition: transform 0.4s ease, z-index 0.3s ease, opacity 0.5s ease;
  z-index: 1;
  opacity: 0;
  transform: translate(0px, 0px) scale(1);
  transform-origin: center;

  &.active {
    left: 0;
    right: 0;
    z-index: 6;
    opacity: 1;
    transform: translate(0px, 0px) scale(1);
  }

  &.active_prev {
    z-index: 2;
    opacity: 1;
    transform: translate(-218px, 0px) scale(0.8);
  }
  &.active_next {
    transform: translate(218px, 0px) scale(0.8);
    z-index: 2;
    opacity: 1;
  }
`;

const Mask = styled.div`
  position: absolute;
  z-index: 4;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
`;

const Carousel: FC<CarouselProps> = props => {
  const [active, setActive] = useState(0);
  const len = useMemo(() => props.list.length, [props.list]);

  const handlePrev = useCallback(() => {
    setActive(preVal => {
      return preVal === len - 1 ? 0 : preVal + 1;
    });
  }, [len]);
  const handleNext = useCallback(() => {
    setActive(preVal => {
      return preVal === 0 ? len - 1 : preVal - 1;
    });
  }, [len]);

  return (
    <CarouselCom className="carousel">
      {props.list.map((item, idx) => {
        const active_prev_class =
          (active == 0 && idx == len - 1) || (active != 0 && idx == active - 1)
            ? "active_prev"
            : "";

        const active_class = idx == active ? "active" : "";

        let active_next_class =
          idx == active + 1 || (active == len - 1 && idx == 0)
            ? "active_next"
            : "";

        return (
          <CarouselItemWrapper
            className={cls(
              "item-wrapper",
              active_class,
              active_prev_class,
              active_next_class
            )}
            key={idx}
          >
            <Item className="item">
              <div className="item-content">
                <img src={item?.picture} alt="" />
                {idx != active && <Mask />}
              </div>
            </Item>
          </CarouselItemWrapper>
        );
      })}
      <div className="prev" onClick={handleNext}>
        {"<"}
      </div>
      <div className="next" onClick={handlePrev}>
        {">"}
      </div>
    </CarouselCom>
  );
};

export default Carousel;
