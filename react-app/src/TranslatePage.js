import React, { useEffect, useState, useRef } from 'react';
import { useHistory } from 'react-router-dom';
import { Box, Button, Card, CardHeader, Divider, Grid } from '@material-ui/core';
import TextareaAutosize from '@mui/material/TextareaAutosize';
import TextField from '@mui/material/TextField';
import Stack from '@mui/material/Stack';
// eslint-disable-next-line import/no-unresolved
import { makeStyles } from '@material-ui/styles';
import Container from '@mui/material/Container';
import Paper from '@mui/material/Paper';

const useStyles = makeStyles(theme => ({
  root: {
    // background: 'linear-gradient(45deg, #FE6B8B 30%, #FF8E53 90%)',
    border: 0,
    borderRadius: 3,
    boxShadow: '0 3px 5px 2px rgba(96, 96, 96, .3)',

    // color: 'white',
    height: '100%',
    padding: '0 30px'
  },
  textAreaStyle: {
    width: '100%',
    border: 'none',
    resize: 'none',
    cursor: 'pointer',
    outline: 'none',
    fontSize: 24,
    autofocus: 'autofocus'
  }
}));

export default () => {
  const { protocol, host } = window.location;
  const classes = useStyles();
  const textArealeft = useRef();
  const [value, setValue] = useState('');
  const [fromLang, setFromLang] = useState('en');
  const [toLang, setToLang] = useState('zh-CN');

  const [times, setTimes] = useState(1);
  const [translateValue, setTranslateValue] = useState('');
  const [translatePlaceholder, setTranslatePlaceholder] = useState('翻译');

  const [wasms, setWsams] = useState();
  const loadWasm = async () => {
    try {
      // eslint-disable-next-line import/no-unresolved
      const wasms1 = await import('react-wasm');
      return wasms1;
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
    return null;
  };

  useEffect(() => {
    const future = loadWasm();
    future.then(resp => {
      if (resp !== null) {
        setWsams(resp);
      }
    });

    console.log('111');
  }, [wasms]);

  const getTranslateSign = value => {
    const url = `${protocol + host}/translate`;
    const dealValue = value.replace(/\r\n/g, '').replace('\n', '');
    wasms.translate(url, dealValue, fromLang, toLang, times).then(val => {
      /*   const d = val.indexOf('\n', 6);
      const e = val.substring(6, d);
      const f = Number(e);
      const c = val.substr(d, f);
      const valueJson = window.JSON.parse(c);
      const jsonStr1 = valueJson[0][2];
      const valueJson2 = window.JSON.parse(jsonStr1);
      const realValueArray = valueJson2[1][0][0][5];
      const realValue = realValueArray.map(item => item[0]);
      console.log(realValue); */
      setTranslateValue(val);
      setTranslatePlaceholder('翻译');
    });
  };

  const onChange = event => {
    const val = event.target.value;

    setValue(val);
    setTranslateValue('');
    if (val !== '') {
      setTranslatePlaceholder('正在翻译....');
      console.time('excute time');
      getTranslateSign(val);
      console.timeEnd('excute time');
    } else {
      setTranslatePlaceholder('翻译');
    }

    console.log(event.target.value);
  };

  return (
    <>
      <Paper>
        <Card>
          <CardHeader title="翻译" sx={{ backgroundColor: '' }} />
          <Divider />
          <Stack
            direction="row"
            divider={<Divider orientation="vertical" flexItem />}
            spacing={1}
            padding={2}
          >
            <TextareaAutosize
              ref={textArealeft}
              minRows={20}
              className={classes.textAreaStyle}
              autofocus
              onChange={onChange}
            />
            <TextareaAutosize
              placeholder={translatePlaceholder}
              value={translateValue}
              className={classes.textAreaStyle}
              minRows={20}
              autofocus
            />
          </Stack>
        </Card>
      </Paper>
    </>
  );
};
